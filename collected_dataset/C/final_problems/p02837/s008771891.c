#include <stdio.h>
int x[16][16], y[16][16];
int pow2[16]={1,2,4,8,16,32,64,128,256,512,1024,
2048,4096,8192,16384,32768};
int binary[16];
int work1(int n)
{
	int i = 1;
	while(n)
	{
		binary[i] = n%2;
		i++;
		n /= 2;
	}
	return i;
}
int work2(int t)
{
	int cnt = 0;
	int flag = 0;          //flag=1表示有冲突了 
	for(int i=t-1;i>=1;i--)
	{
		if(binary[i]==1)    //此次讨论情况中假设了第i个人诚实
		{
			cnt++;
			for(int j=1;x[i][j]!=0;j++)  //则他说的话都是真的 
			{
				if(y[i][j]==1)    //第i个人说x[i][j]诚实 
				{
					if(binary[x[i][j]]!=1)      //看看是不是真诚实 
					{
						flag = 1;
						break;
					}
				}
				else
				{
					if(binary[x[i][j]]!=0)
					{
						flag = 1;
						break;
					}
				}
				if(flag) break;
			}
		}
		else
		{
			for(int j=1;x[i][j]!=0;j++)
			{
				if(y[i][j]==1)
				{
					if(binary[x[i][j]]==1)
					{
						flag = 1;
						break;
					}
				}
				else
				{
					if(binary[x[i][j]]==0)
					{
						flag = 1;
						break;
					}
				}
				if(flag) break;
			}
		}
		if(flag) break;
	}
	if(!flag) return cnt;
	else return 0;
}
int main()
{
	int n, ans = -1;
	scanf("%d", &n);
	for(int i=1;i<=n;i++)
	{
		int t;
		scanf("%d", &t);
		for(int j=1;j<=t;j++)
		{
			scanf("%d %d", &x[i][j], &y[i][j]);
		}
	}
	for(int i=1;i<=pow2[n];i++)
	{
		int t = work1(i);     //把i换成2进制数
		int temp = work2(t);
		if(temp>ans) ans = temp;
	}
	printf("%d\n", ans);
	return 0;
}