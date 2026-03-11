#include <stdio.h>
int Rt[200000], max[200000];

int main()
{
	
	int n, i, a, b, c;
	scanf("%d ",&n);
	for (int i = 0; i < 199999; i++)
	{
		Rt[i] = -10000000;
		max[i] = -10000000;
	}

	for (i = 0; i < n; i++)
	{
		scanf("%d", &Rt[i]);
	}


	for (i = 0; i < n; i++) // 选定下标为i的数字
	{
		for (int j = i + 1; j < n; j++) // 选定其后面的数与Rt[i]做差
		{
			a = Rt[j] - Rt[i]; // a 为差值
			if (j == i + 1)
				b = a; // 将第一个差值存进b
			else if (b >= a)
				continue;
			else b = a; // b 存储（以Rt[i]为首的）最大差值
		}
		max[i] = b;
	}
	c = max[0];
	for (i = 1; i < n; i++) // 比较各个最大差值，选出最大值
	{
		if (c >= max[i + 1])
			continue; // c 存储差值中的最大值
		else c = max[i + 1];
	}
	printf("%d", c);
	getchar(); 
	getchar();
	return 0;
}
