import sqlite3


def main():
    conn = sqlite3.connect('.coverage')
    cur = conn.cursor()

    display_table_metadata(cur)
    print()
    show_meta_table(cur)
    print()
    show_file_table(cur)
    print()
    show_context_table(cur)
    print()
    show_line_bits_table(cur)
    print()
    show_arc_table(cur)
    print()
    show_tracer_table(cur)


def display_table_metadata(cur):
    response = cur.execute("SELECT * FROM sqlite_master WHERE type='table';")
    for item in response.fetchall():
        print(item[4])
        print()


def show_meta_table(cur):
    print('Meta Table')
    _print_table(cur, 'meta')


def show_file_table(cur):
    print('File Table')
    _print_table(cur, 'file')


def show_context_table(cur):
    print('Context Table')
    _print_table(cur, 'context')


def show_line_bits_table(cur):
    print('Line Bits Table')
    _print_table(cur, 'line_bits')


def show_arc_table(cur):
    print('Arc Table')
    _print_table(cur, 'arc')


def show_tracer_table(cur):
    print('Tracer Table')
    _print_table(cur, 'tracer')


def _print_table(cur, table_name):
    response = cur.execute(f'SELECT * FROM {table_name}')
    for item in response.fetchall():
        print(item)


# context
# line bits
# arc
# tracer

if __name__ == "__main__":
    main()

