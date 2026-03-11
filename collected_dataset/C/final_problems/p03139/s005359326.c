#include <stdio.h>
#define max(x,y) ((x)>(y)?(x):(y))
#define min(x,y) ((x)<(y)?(x):(y))
#define FOR(i,m,n) for(i=m;i<n;i++)

int main(void)
{
	int a,b,c,d;
	scanf("%d %d %d",&a,&b,&c);
	if(a-b-c<0){
		d=b+c-a;
	}else{
		d=0;
	}
	printf("%d %d\n",min(b,c),d);
	return 0;
}