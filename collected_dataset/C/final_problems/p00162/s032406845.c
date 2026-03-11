#include <stdio.h>

int main(){
	int m,n,i,j,k,cnt,t;
	while(1){
		scanf("%d %d",&m,&n);
		if(m==0) break;
		cnt=0;
		for(i=m;i<=n;i++){
			t=i;
			for(;;t/=2) if(t%2!=0) break;
			for(;;t/=3) if(t%3!=0) break;
			for(;;t/=5) if(t%5!=0) break;
			if(t==1) cnt++;
		}
		printf("%d\n",cnt);
	}
	return 0;
}