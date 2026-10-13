# comparationpython.py

def sum_of_squares(numbers):
    """Calcula a soma dos quadrados dos números."""
    return sum(x * x for x in numbers)


def filter_even(numbers):
    """Filtra os números pares e retorna em uma nova lista."""
    return [x for x in numbers if x % 2 == 0]


def main():
    # Cria uma lista com números de 1 até 1.000.000
    numbers = list(range(1, 10_000_001))

    # Calcula a soma dos quadrados de forma sequencial
    sum_squares = sum_of_squares(numbers)
    print("Soma dos quadrados: ", sum_squares)

    # Filtra em Números pares de forma sequencial
    even_numbers = filter_even(numbers)
    print("Quantidade de números pares: ", len(even_numbers))

if __name__ == "__main__":
    main()
