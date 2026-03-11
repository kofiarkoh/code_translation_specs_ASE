#include <stdio.h>
int main() {
	int n;
	long int ini[5] = {}, ans1 = 0, ans2 = 0;
	char name[11];
	scanf("%d", &n);
	for (int i = 0; i < n; i++)
	{
		scanf("%s", name);
		switch (name[0])
		{
		case 'M':
			ini[0]++;
			break;
		case 'A':
			ini[1]++;
			break;
		case 'R':
			ini[2]++;
			break;
		case 'C':
			ini[3]++;
			break;
		case 'H':
			ini[4]++;
			break;
		default:
			break;
		}
	}
	for (int i = 0; i < 3; i++)
	{
		for (int j = i + 1; j < 4; j++)
		{
			for (int k = j + 1; k < 5; k++)
			{
				ans2 += ini[i] * ini[j] * ini[k];
				ans1 += ans2 / 10000000;
				ans2 %= 10000000;
			}
		}
	}
	if(ans1){
		printf("%lu%07lu", ans1, ans2);
	}
	else{
		printf("%lu", ans2);
    }
	return 0;
}