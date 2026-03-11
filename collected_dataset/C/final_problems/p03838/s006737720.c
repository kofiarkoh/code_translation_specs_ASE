#include<stdio.h>
#include <string.h>
int main()
{
	int x,y,xa,ya,i=0;
	
	scanf("%d %d",&x,&y);
	
	if(x>=0 && y>=0){
		xa=x;
		ya=y;
		
		if(xa>ya){
			i=2+xa-ya;
		}else if(xa<ya){
			i=ya-xa;
		}else{
			i=0;
		}
	}else if(x<0 && y>=0){
		xa=-1*x;
		ya=y;
		if(xa>ya){
			i=xa-ya+1;
		}else if(xa<ya){
			i=1+ya-xa;
		}else{
			i=1;
		}
	}else if(x<0 && y<0){
		xa=-1*x;
		ya=-1*y;
		if(xa>ya){
			i=xa-ya+1;
		}else if(xa<ya){
			i=ya-xa+2;
		}else{
			i=0;
		}
	}else{
		xa=x;
		ya=-1*y;
		if(xa>ya){
			i=1+xa-ya;
		}else if(xa<ya){
			i=ya-xa+1;
		}else{
			i=1;
		}
	}
	
	printf("%d\n",i);
	
	return 0;
}
