#include<stdio.h>
int main(void)
{
	int D[7],i,j,n,t,f,T,F;
	for(i=1;i<=6;i++){scanf("%d",&D[i]);}
	scanf("%d",&n);
	for(i=0;i<n;i++)
	{
		scanf("%d %d",&T,&F);
		for(j=1;j<=6;j++){if(T==D[j])t=j;}
		for(j=1;j<=6;j++){if(F==D[j])f=j;}
		switch(t){
			case 1:switch(f){
				case 2:printf("%d\n",D[3]);break;
				case 3:printf("%d\n",D[5]);break;
				case 4:printf("%d\n",D[2]);break;
				case 5:printf("%d\n",D[4]);break;
			}break;
			case 2:switch(f){
				case 1:printf("%d\n",D[4]);break;
				case 3:printf("%d\n",D[1]);break;
				case 4:printf("%d\n",D[6]);break;
				case 6:printf("%d\n",D[4]);break;
			}break;
			case 3:switch(f){
				case 1:printf("%d\n",D[2]);break;
				case 2:printf("%d\n",D[6]);break;
				case 5:printf("%d\n",D[1]);break;
				case 6:printf("%d\n",D[5]);break;
			}break;
			case 4:switch(f){
				case 1:printf("%d\n",D[5]);break;
				case 2:printf("%d\n",D[1]);break;
				case 5:printf("%d\n",D[6]);break;
				case 6:printf("%d\n",D[2]);break;
			}break;
			case 5:switch(f){
				case 1:printf("%d\n",D[3]);break;
				case 3:printf("%d\n",D[6]);break;
				case 4:printf("%d\n",D[1]);break;
				case 6:printf("%d\n",D[4]);break;
			}break;
			case 6:switch(f){
				case 2:printf("%d\n",D[4]);break;
				case 3:printf("%d\n",D[2]);break;
				case 4:printf("%d\n",D[5]);break;
				case 5:printf("%d\n",D[3]);break;
			}break;
		}
	}
	return 0;
}