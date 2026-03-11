#include<stdio.h>
int main(void)
{
		int n,t,i,k,j,g,s,r,a,w,z,o;
	while(1)	{
		scanf("%d",&t);
		if(t==-1)	break;
		if(10>=t){
			printf("3130\n");
			}
		if(t>10&&t<20){
			n=t-10;
			i=4280-(n*125+1150);
			printf("%d\n",i);
			}
		if(t>20&&t<30){
			k=t-20;
			j=4280-((k*140)+2400);
			printf("%d\n",j);
		}
		if(t>30){
			s=t-30;
			g=4280-((s*160)+3800);
			printf("%d\n",g);
		}	
	}
	return 0;
}