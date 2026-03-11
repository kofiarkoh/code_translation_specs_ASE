#include <stdio.h>
int main()
{
	int s[1000000];
	int n; //ループ管理用
	int yon; //4になったときの数字
	scanf ("%d", &s[0]); //初項の入力
	
	for(n=0; n<1000000; n++)
	{
		if(s[n] % 2 == 0) s[n+1] = s[n] / 2;
		else s[n+1] = 3*s[n] + 1;
		
		if(s[n+1] == 4){
			yon = n+1;
			break;
		}
		
	}
	
	printf("%d", yon+4 );
	return 0;
}