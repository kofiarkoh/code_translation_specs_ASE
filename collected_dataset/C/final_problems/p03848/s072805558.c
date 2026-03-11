#include<stdio.h>
#include<stdbool.h>
#include<math.h>
long long f[100002];

int main() {
	long long N,i,A, tmp = 0, ans = 0;;
	scanf("%lld", &N);
	bool even = true;
	for (i = 0; i < N; i++)
	{
		scanf("%lld", &A);
		f[A]++;
		//成立しているかチェックする
		if (i == 0 && A % 2 == 1)//f[0]が奇数の場合、全てが奇数か？
		{
			even = true;
		}
		else if(i == 0)
		{
			even = false;
		}
		if (even == true) {
			if (A % 2 != 1)//全てが奇数ではなかった場合
			{
				//ありうる並び方がないので０を出力する
				puts("0");
				return 0;
			}
			else//今のところ全てが奇数の場合
			{
				if (f[A] == 2)//２つあったものを数える
				{
					tmp++;
				}
				if (i == N - 1 && N / 2 == tmp)//最終的に全て２つあった場合
				{
					ans = (int)pow(2, ((double)N / 2))% 1000000007;
					printf("%lld\n", ans);
					return 0;
				}
			}
		}
		if (even != true) {
			if (A % 2 == 1 && A != 0)//ゼロで無い全てのものが偶数ではなかった場合
			{
				//ありうる並び方がないので０を出力する
				puts("0");
				return 0;
			}
			else if (f[0] > 1) //ゼロの発生回数が１つ以下ではなかった場合
			{
				//ありうる並び方ではないので０を出力する
				puts("0");
				return 0;
			}
			else//ゼロの発生回数が１つ以下で今のところゼロで無い全てのもの全てが偶数の場合
			{
				if (f[A] == 2)//２つあったものを数える
				{
					tmp++;
				}
				if (f[0] != 1 && i == N - 1)//すべてが数え終わったときゼロであったものは一つではなかった場合
				{
					//ありうる並び方ではないので０を出力する
					puts("0");
					return 0;
				}
				else if (i == N - 1 && (N - 1) / 2 == tmp && (N - 1) % 2 == 0 && f[0] == 1)//最終的にゼロで無い全てのもの全てが偶数かつゼロであったものは一つだった場合
				{
					ans = (int)pow(2, ((double)(N - 1) / 2)) % 1000000007;
					printf("%lld\n", ans);
					return 0;
				}
			}
		}
	}
	return 0;
}
