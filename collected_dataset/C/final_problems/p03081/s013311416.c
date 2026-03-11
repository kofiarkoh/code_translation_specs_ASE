#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>

typedef struct Hoge{
	int golem;
	char c;
}hoge;

int main(void){
	hoge masu[210000];
	int n,q;
	char temp[210000];
	scanf("%d %d",&n,&q);
	scanf("%s",temp);

	int i,j;
	for(i = 1;i<=n;i++){
		masu[i].golem = 1;
		masu[i].c = temp[i-1];
	}
	for(i=0;i<q;i++){
		char t[5],d[5];
		scanf("%s %s",t,d);
		for(j=1;j<=n;j++){
			if(masu[j].c == t[0]){				
				if(d[0] == 'L'){
					masu[j-1].golem += masu[j].golem;
					masu[j].golem = 0;
				}else{
					masu[j+1].golem += masu[j].golem;
					masu[j].golem = 0;
				}
			}
		}
	}

	int ans=0;
	for(i = 1;i<=n;i++){
		ans += masu[i].golem;
	}
	printf("%d\n",ans);
	return 0;


}
