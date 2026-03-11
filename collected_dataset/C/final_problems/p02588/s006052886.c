#include<stdio.h>
#include<string.h>
#include<math.h>

#define DECIMAL_POINT 9
#define UPPER_DECIMAL 0
#define UNDER_DECIMAL -1

#define element_num_max 19

long long int array[200001];

int main()
{
	int n = 0;
	long long int t;
	int d_p, d_c;
	long long int mul = 0;
	long long int answer = 0;
	char num_arrray[20];
	int ele_two_ind, ele_five_ind;

	int element_array[element_num_max][element_num_max];

	for (int i = 0; i<element_num_max; i++)
	{
		for (int j = 0; j<element_num_max; j++)
		{
			element_array[i][j] = 0;
		}
	}

	scanf("%d", &n);

	for (int i = 0; i<n; i++)
	{
		scanf("%s", num_arrray);

		d_p = DECIMAL_POINT;
		d_c = UPPER_DECIMAL;
		t = 0;

		for (int j = 0; j<strlen(num_arrray); j++)
		{
			int current_num = 0;

			if (num_arrray[j] == '.')
			{
				d_c = UNDER_DECIMAL;
				continue;
			}

			current_num = num_arrray[j] - '0';

			if (d_c == UPPER_DECIMAL)
			{
				t = t * 10 + current_num;
			}
			else {
				t = t * 10 + current_num;
				d_p += UNDER_DECIMAL;
			}

		}

		for (int k = 0; k<d_p; k++)
		{
			t *= 10;
		}

		array[i] = t;

		for (ele_two_ind = 0; (t % 2) == 0; ele_two_ind++)
		{
			t /= 2;
		}

		if (ele_two_ind >= element_num_max)
			ele_two_ind = (element_num_max - 1);

		for (ele_five_ind = 0; (t % 5) == 0; ele_five_ind++)
		{
			t /= 5;
		}

		if (ele_five_ind >= element_num_max)
			ele_five_ind = (element_num_max - 1);

		element_array[ele_two_ind][ele_five_ind] ++;

	}

	for (int i = 0; i<element_num_max; i++)
	{
		for (int j = 0; j<element_num_max; j++)
		{
			for (int i_s = (element_num_max - 1 - i); (i_s<element_num_max); i_s++)
			{
                int j_s = (element_num_max - 1 - j);

				if (i > i_s)
					i_s = i;

                if (i == i_s)
                {
                    j_s = j;
                }

				for (; (j_s<element_num_max); j_s++)
				{
					if ((i == i_s) && (j == j_s))
					{
						answer += element_array[i][j] * (element_array[i_s][j_s] - 1) / 2;
					}
					else {
						answer += element_array[i][j] * element_array[i_s][j_s];
					}
				}
			}
		}
	}

	printf("%lld", answer);

	return 0;
}