#include <stdio.h>

int main()
{
	int W,H,a[1000],b[1000];
	int i,j,k,tmp;
	int sum_w = 0;
	int sum_h = 0;
	int result;

	scanf("%d %d", &W, &H);//横、高さ
	for (i = 0; i < W; i++)
	{
		scanf("%d", &a[i]);
		sum_w += a[i];
	}
	for (i = 0; i < H; i++)
	{
		scanf("%d", &b[i]);
		sum_h += b[i];
	}
	if (sum_w != sum_h)
	{
		result = 0;
	}
	//↑縦と横の合計が一致しているかのチェック
	else//↓のアルゴリズムが理解できない
	{
		result = 1;
		for (i = 0; i < W; i++)//横
		{
			for (j = 0; j < H; j++)//高さ
			{
				for (k = j; k < H; k++)
				{
					if (b[j] < b[k])//bを昇順にする？？
					{
						tmp = b[j];
						b[j] = b[k];
						b[k] = tmp;
					}
				}
			}
			for (j = 0; j < H; j++)
			{
				if (a[i] == 0 || b[j] == 0)
				{
					break;
				}
				a[i]--;
				b[j]--;
			}
			if (a[i] > 0)
			{
				result = 0;
				break;
			}
		}
	}
	printf("%d\n", result);
}



