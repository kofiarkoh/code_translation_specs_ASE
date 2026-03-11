#include<stdio.h>
int main(void)
{
	int n,h,w;
	scanf("%d %d %d",&n,&h,&w);

    if(n==w || n==h){
        printf("%d", h);
    }
    else if(h==w && w!=n){
        printf("%d", n*n);
    }
    else if(n==h==w){
        printf("%d", n/n);
    }
    else printf("%d", h*w);
	return 0;
}