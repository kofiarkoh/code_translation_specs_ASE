#include <stdio.h>
#define max(x,y) ((x)>(y)?(x):(y))
#define min(x,y) ((x)<(y)?(x):(y))
#define FOR(i,m,n) for(i=m;i<n;i++)

int main(void)
{
	int a,b=0,f,i;
	char c[101],d[101],e[101];
	scanf("%d %s %s %s",&a,&c,&d,&e);
	for(i=0;i<a;i++){
		if(c[i]==d[i]&&d[i]==e[i]){
			b=b;
		}else if(c[i]!=d[i]&&d[i]!=e[i]&&c[i]!=e[i]){
			b=b+2;
		}else if(c[i]==d[i]){
			b=b+1;
		}else if(d[i]==e[i]){
			b=b+1;
		}else{
			b=b+1;
		}
	}
	printf("%d\n",b);
	return 0;
}