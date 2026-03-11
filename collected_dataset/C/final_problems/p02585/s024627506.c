#include<stdio.h>
#include<string.h>

long long int p_array[5001];
long long int c_array[5001];


int main()
{
	long long int score = -1000000001, n, k;
	long long int current_score;

	scanf("%lld %lld", &n, &k);

	for (int i = 1; i <= n; i++)
	{
		scanf("%lld", &p_array[i]);
	}

	for (int i = 1; i <= n; i++)
	{
		scanf("%lld", &c_array[i]);
	}

	for (int i = 1; i <= n; i++)
	{
		int first_step = i;
		int next_step = p_array[first_step];
		long long int step_num = 0;
		current_score = 0;

		while ((first_step != next_step) && (step_num < k))
		{
			current_score += c_array[next_step];
			step_num++;

			next_step = p_array[next_step];

			if (score < current_score)
			{
				score = current_score;
			}

			if ((first_step == next_step) && (step_num < k))
			{
				current_score += c_array[next_step];
				step_num++;
			}


			if (score < current_score)
			{
				score = current_score;
			}
		}


		if ((current_score > 0) && (step_num < k))
		{
			long long int round_scoure = 0;
			round_scoure = ((k - step_num) / step_num)*current_score;

			first_step = i;
			next_step = p_array[first_step];;

			for (long long int j = 0; j<((k%step_num) + step_num); j++)
			{
				round_scoure += c_array[next_step];
				next_step = p_array[next_step];

				if (round_scoure > score)
				{
					score = round_scoure;
				}
			}

		}

	}


	printf("%lld", score);

	return 0;
}