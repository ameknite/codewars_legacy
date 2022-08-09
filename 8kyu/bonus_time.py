def bonus_time(salary: int, bonus: bool) -> str:
    return f"${salary * (10 if bonus else 1)}"
