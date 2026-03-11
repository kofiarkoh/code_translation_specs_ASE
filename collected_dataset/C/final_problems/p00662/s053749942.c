#include <stdio.h>

int main(void) {
	int n1,n2,n3,n4,n5,n6;
	int c1,c2,c3;
	int t;
	while(1) {
		scanf("%d%d%d%d%d%d",
			&n1,&n2,&n3,&n4,&n5,&n6);
		if((n1|n2|n3|n4|n5|n6)==0)break;
		c1=n1+n4;c2=n2+n5;c3=n3+n6;
		t=c1;
		if(c2<t)t=c2;
		if(c3<t)t=c3;
		c1-=t;c2-=t;c3-=t;
		printf("%d\n",t+c1/3+c2/3+c3/3);
	}
	return 0;
}