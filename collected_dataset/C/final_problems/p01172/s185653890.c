#include <stdio.h>
#include <string.h>

int amari[1000000];

int main(void) {
	int x,y;
	int i;
	while(1) {
		scanf("%d%d",&x,&y);
		if(x==0 && y==0)break;
		memset(amari,0,sizeof(amari));
		amari[x]=1;
		for(i=2;;i++) {
			x=(x*10)%y;
			if(amari[x]>0) {
				printf("%d %d\n",amari[x]-1,i-amari[x]);
				break;
			} else if(x==0) {
				printf("%d 0\n",i-1);
				break;
			}
			amari[x]=i;
		}
	}
	return 0;
}