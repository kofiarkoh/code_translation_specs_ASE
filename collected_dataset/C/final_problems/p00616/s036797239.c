#include<stdio.h>

char cube[500][500][500];

int main(){
	int i,j,k,a,b,n,h,t;
	char c[2];
	t=0;
	while(0<=scanf("%d%d",&n,&h)){
		if(n==0) break;
		++t;
		for(i=0;i<h;i++){
			scanf(" %c%c ",&c[0],&c[1]);
			scanf("%d%d",&a,&b);
//			printf("i=%d %c%c %d %d\n",i,c[0],c[1],a,b);
			if(c[0]=='x' && c[1]=='y'){
				for(j=0;j<n;j++){
					cube[a-1][b-1][j]=t;
				}
			}else if(c[0]=='x' && c[1]=='z'){
				for(j=0;j<n;j++){
					cube[a-1][j][b-1]=t;
				}
			}else{
				for(j=0;j<n;j++){
					cube[j][a-1][b-1]=t;
				}
			}
		}
		a=0;
		for(i=0;i<n;i++){
			for(j=0;j<n;j++){
				for(k=0;k<n;k+=3){
					if(cube[i][j][k]!=t) a++;
					if(k+1<n)if(cube[i][j][k+1]!=t) a++;
					if(k+2<n)if(cube[i][j][k+2]!=t) a++;
				}
			}
		}
		printf("%d\n",a);
	}
			
	return 0;
}