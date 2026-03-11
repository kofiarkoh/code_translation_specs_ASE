#include<stdio.h>

int main(void){
	int n,m,pcnt=0,ccnt=0,t=0,tem=0;
	scanf("%d%d",&n,&m);
	int p[m];
	char s[m][3];
	int a[m];
	for(int i=0;i<m;i++){
		scanf("%d %s",&p[i],&s[i]);
	}
	for(int i=0;i<m;i++){
		if(s[i][0]=='A'){
			for(int k=0;k<=t;k++){
				if(a[k]==p[i]){
					tem=1;
				}
			}
			if(tem!=1){
				ccnt++;
				a[t]=p[i];
				t++;
				if(t==n){
					break;
				}
			}

		}else{
			for(int k=0;k<=t;k++){
				if(a[k]==p[i]){
					tem=1;
			  }
			}
			if(tem!=1){
				pcnt++;
			}
			tem=0;
		}

	}
	printf("%d %d",ccnt,pcnt);
	return 0;
}
