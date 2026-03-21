import asyncio


async def task_1():
    for i in range(10):
        print(f'Task 1: {i}')
        await asyncio.sleep(1)

    return "Done with Task 1"

async def task_2():
    for i in range (5):
        print(f'Task 2: {i}')
        await asyncio.sleep(2)

    return "Done with Task 2"

async def task_3():
    await asyncio.sleep(1)
    return "Done with Task 3"


async def main():
    task1 = asyncio.create_task(task_1())

    val2 = await task_2()
    val1 = await task1

    # Run after task 1 and task 2 due to the awaits
    results3 = await asyncio.gather(task_3(), task_3(), task_3())

    print('\n\n')

    print(val1)
    print(val2)
    print(results3)


if __name__ == "__main__":
    asyncio.run(main())
