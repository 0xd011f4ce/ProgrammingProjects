#include <stdio.h>

int
main (void)
{
	int num = 0;
	printf ("Please enter a three digits number: ");
	scanf ("%d", &num);

	int digit_unit = (num % 10);
	int digit_ten = (num % 100) / 10;
	int digit_hundred = (num % 1000) / 100;

	printf ("%d + %d + %d = %d\n", digit_hundred, digit_ten, digit_unit,
					digit_unit + digit_ten + digit_hundred);

	int reversed_number = digit_unit;
	reversed_number = (reversed_number * 10) + digit_ten;
	reversed_number = (reversed_number * 10) + digit_hundred;

	printf ("The reversed number is: %d\n", reversed_number);

	return 0;
}
