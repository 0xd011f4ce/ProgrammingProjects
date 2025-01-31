#include <stdio.h>

int
main (void)
{
	int num1 = 0, num2 = 0, num3 = 0;
	printf ("Enter the two numbers: ");
	scanf ("%d %d", &num1, &num2);

	printf ("%s\n", num1 == num2 ? "EQUAL" : "NOT EQUAL");

	printf ("Now, enter 3 numbers: ");
	scanf ("%d %d %d", &num1, &num2, &num3);

	printf ("%s\n", num1 == num2 && num2 == num3 ? "EQUAL" : "NOT EQUAL");

	printf ("Enter a three digit number: ");
	scanf ("%d", &num1);

	int digit1 = (num1 % 1000) / 100;
	int digit2 = (num1 % 100) / 10;
	int digit3 = (num1 % 10);

	printf ("%s\n",
					digit1 < digit2 && digit2 < digit3 ? "ASCENDING" : "NOT ASCENDING");

	printf ("Enter a value that can be eihter positive or negative: ");
	scanf ("%d", &num1);

	printf ("%d\n", num1 > 0 ? 1 : -1);

	return 0;
}
