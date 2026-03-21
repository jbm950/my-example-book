from collections import UserList

from pydantic import BaseModel, field_validator


class CustomList(UserList):
    pass


class ListItem(BaseModel):
    id: int


class TopModel(BaseModel):
    name: str
    custom_list: list[ListItem]

    @field_validator('custom_list', mode='after')
    def build_custom_list(cls, v):
        return CustomList(v)


def main():
    data = {'name': 'cool name',
            'custom_list': [{'id': 1}, {'id': 2}, {'id': 3}]}
    print(TopModel(**data))


if __name__ == "__main__":
    main()
