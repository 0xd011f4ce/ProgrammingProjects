#include <stdio.h>

int
main (void)
{
	int op1 = 0, op2 = 0, result = 0;
	char op = '\0';

	printf ("Enter the operation to do ('+', '-', '*', '/', '%%') i.e. 2 + 5: ");
	scanf ("%d %c %d", &op1, &op, &op2);

	switch (op)
		{
		case '+':
			result = op1 + op2;
			break;

		case '-':
			result = op1 - op2;
			break;

		case '*':
			result = op1 * op2;
			break;

		case '/':
			if (op2 == 0)
				{
					printf ("The second operand cannot be zero. Aborting.\n");
					break;
				}

			result = op1 / op2;
			break;

		case '%':
			if (op2 == 0)
				{
					printf ("The second operand cannot be zero. Aborting.\n");
					break;
				}

			result = op1 % op2;
			break;

		default:
			printf ("The operation %c is not defined.\n", op);
			break;
		}

	printf ("The result of %d %c %d is %d\n", op1, op, op2, result);

	return 0;
}
