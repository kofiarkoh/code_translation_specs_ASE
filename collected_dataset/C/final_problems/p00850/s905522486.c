#include <stdio.h>
#include <math.h>
#include <stdlib.h>

#define max_ans 13

int num;
int cache[max_ans + 1];

int dfs(int depth, int max_depth)
{
	if (cache[depth] == num) // correct ans
		return 1;
	if (depth == max_depth) // reach max depth,break;
		return 0;
	int max_value = cache[0];
	for (int i = 1; i <= depth; i++) // find current max power
	{
		max_value = (((max_value) > (cache[i])) ? (max_value) : (cache[i]));
	}
	if ((max_value << (max_depth - depth)) < num) //return false if max_value*2^(left depth) still can't reach num.
		return 0;                                 //which means impossible to reach target num
	for (int i = depth; i >= 0; i--)
	{
		cache[depth + 1] = cache[depth] + cache[i];
		if (dfs(depth + 1, max_depth) == 1)
			return 1;
		cache[depth + 1] = cache[depth] - cache[i];
		if (dfs(depth + 1, max_depth) == 1)
			return 1;
	}
	return 0;
}

int main(int argc, char const *argv[])
{
	for (int input, output; scanf("%d", &input) && input != 0; printf("%d\n", output))
	{
		num = input;
		output = -1;
		if (input == 1)
			output = 0;
		else
		{
			cache[0] = 1;
			for (int i = 0; i < max_ans; i++)
			{
				if (dfs(0, i) == 1)
				{
					output = i;
					goto b1;
				}
			}
			if (output == -1)
				output = max_ans;
		b1:
			output = output;
		}
	}
	return 0;
}

