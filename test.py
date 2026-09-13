import asyncio
import time

import httpx


BASE_URL = "http://localhost:3000"

USER_COUNT = 10
REQUESTS_PER_USER = 500


async def signup(client: httpx.AsyncClient, account: str, password: str):
    response = await client.post(
        f"{BASE_URL}/service/auth/signup",
        json={
            "account": account,
            "password": password,
            "confirmPassword": password,
        },
    )

    if response.status_code not in (200, 201):
        print(f"[SIGNUP] {account}: {response.status_code} {response.text}")
        return False

    return True


from typing import Optional

async def signin(
    client: httpx.AsyncClient,
    account: str,
    password: str,
) -> Optional[str]:
    response = await client.post(
        f"{BASE_URL}/service/auth/signin",
        json={
            "account": account,
            "password": password,
        },
    )

    if response.status_code != 200:
        print(f"[SIGNIN] {account}: {response.status_code} {response.text}")
        return None

    return response.json()["token"]


async def create_todo(
    client: httpx.AsyncClient,
    account: str,
    token: str,
    number: int,
):
    response = await client.put(
        f"{BASE_URL}/service/todo",
        headers={
            "Authorization": f"Bearer {token}",
        },
        json={
            "dueDate": "2026-09-12T00:00:00Z",
            "title": f"{account} todo {number}",
            "description": f"load test {account} {number}",
        },
    )

    return response


async def run_user(
    client: httpx.AsyncClient,
    account: str,
    password: str,
):
    token = await signin(client, account, password)

    if token is None:
        return

    tasks = [
        create_todo(client, account, token, i)
        for i in range(REQUESTS_PER_USER)
    ]

    responses = await asyncio.gather(*tasks)

    success = sum(1 for r in responses if 200 <= r.status_code < 300)

    print(
        f"[USER] {account}: "
        f"{success}/{REQUESTS_PER_USER} succeeded"
    )


async def main():
    users = [
        (f"loadtest{i:02d}", "P@55w0rd")
        for i in range(1, USER_COUNT + 1)
    ]

    limits = httpx.Limits(
        max_connections=200,
        max_keepalive_connections=200,
    )

    async with httpx.AsyncClient(limits=limits) as client:
        # ユーザー作成
        await asyncio.gather(
            *(
                signup(client, account, password)
                for account, password in users
            )
        )

        # 各ユーザーでサインイン → 10リクエスト
        start = time.perf_counter()

        await asyncio.gather(
            *(
                run_user(client, account, password)
                for account, password in users
            )
        )

        elapsed = time.perf_counter() - start

    print()
    print(f"Total requests: {USER_COUNT * REQUESTS_PER_USER}")
    print(f"Elapsed: {elapsed:.3f} sec")
    print(
        f"Throughput: "
        f"{USER_COUNT * REQUESTS_PER_USER / elapsed:.2f} req/sec"
    )


if __name__ == "__main__":
    asyncio.run(main())
