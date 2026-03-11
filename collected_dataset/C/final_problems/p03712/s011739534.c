#include <stdio.h>



int main(){
	int h,w;
	scanf("%d %d\n",&h,&w);
	for(int i=0;i<w+2;++i)
	{
		printf("#");
	}
	printf("\n");
	for(int j=0;j<h;++j)
	{
		printf("#");
		char width[w];
		gets(width);
		printf("%s",width);
		printf("#\n");	
	}
	for(int i=0;i<w+2;++i)
	{
		printf("#");
	}
	printf("\n");
}
