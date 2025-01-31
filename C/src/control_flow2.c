#include <stdio.h>

int
main (void)
{
	int num1 = 0, num2 = 0, num3 = 0;

	printf ("Enter the three numbers separated by a space: ");
	scanf ("%d %d %d", &num1, &num2, &num3);

	int min = num1, max = num1;
	if (num1 < num2)
		{
			min = num1;
			max = num2;
		}

	if (max < num3)
		{
			max = num3;
		}
	if (num3 < min)
		{
			min = num3;
		}

	printf ("The minimum is %d, and the maximum is %d\n", min, max);

	return 0;
}
