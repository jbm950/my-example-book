import asyncio


async def task_1():
    for i in range(10):
        print(f'Task 1: {i}')
        await asyncio.sleep(1)

async def task_2():
    for i in range (5):
        print(f'Task 2: {i}')
        await asyncio.sleep(2)


async def main():
    task1 = asyncio.create_task(task_1())
    task2 = asyncio.create_task(task_2())

    await task1
    await task2


if __name__ == "__main__":
    asyncio.run(main())
