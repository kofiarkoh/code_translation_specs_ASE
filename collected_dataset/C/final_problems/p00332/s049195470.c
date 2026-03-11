
#include <stdio.h>

int main(void)
{
	int E,Y,c;
	scanf("%d %d",&E,&Y);
	if(E==0){
		if(Y>=1868 && Y<=1911){
			c=Y-1868;
			printf("M%d\n",c+1);
		}
		else if(Y>=1912 && Y<1926){
			c=Y-1912;
			printf("T%d\n",c+1);
		}
		else if(Y>=1926 && Y<1989){
			c=Y-1926;
			printf("S%d\n",c+1);
		}
		else if (Y>=1989 && Y<=2016){
			c=Y-1989;
			printf("H%d\n",c+1);
		}
	}
	else if(E==1)printf("%d\n",1868+Y-1);
	else if(E==2)printf("%d\n",1912+Y-1);
	else if(E==3)printf("%d\n",1926+Y-1);
	else if(E==4)printf("%d\n",1989+Y-1);
	return 0;
}
