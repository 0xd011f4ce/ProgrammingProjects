#include <stdio.h>

int
main (void)
{
	int num1 = 0, num2 = 0;
	printf ("Please enter num1 and num2: ");
	scanf ("%d %d", &num1, &num2);

	if (num1 > num2)
		{
			printf ("num1 is greater with a value of %d\n", num1);
		}
	else
		{
			printf ("num2 is greater with a value of %d\n", num2);
		}

	return 0;
}
