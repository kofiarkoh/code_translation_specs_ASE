#include <stdio.h>

int main()
{
	int n[4];
	int ans = 1;
	for (int i = 0; i < 4; ++i)
	{
		scanf("%d",&n[i]);
		if(n[i]!=1&&n[i]!=9&&n[i]!=7&&n[i]!=4){
			ans = 0;
		}
	}
	for (int i = 0; i < 4; ++i)
	{
		for (int j = 0; j < 4; ++j)
		{
			if(i!=j&&n[i]==n[j]){
				ans=0;
			}
		}
	}
	if(ans==1){
		printf("YES");
	}else{
		printf("NO");
	}
	return 0;
}
