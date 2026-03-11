#include <stdio.h>

int main(void)
{
	int N,Q,i,j,l,m,a,c;
	int k[75],check[2]={0},cnt;
	char S[256];
	
	scanf("%d",&N); //文字列の長さ
	scanf("%s",&S); //文字列
	scanf("%d",&Q); //kの個数
	
	for(i=0;i<Q;i++){
		cnt=0;
		scanf("%d",&k[i]);
			for(j=0;j<N;j++){
				if(S[j]=='D'){
					a=j;
					check[0]=1;
					for(l=a;l<N;l++){
						if(S[l]=='M' && check[0]==1){
							check[1]=1;
							for(m=l;m<N;m++){
								if(S[m]=='C' && check[1]==1){
									c=m;
									if((c-a)<k[i]){
										cnt+=1;
									}
								}
							}
							check[1]=0;
						}
					}
					check[0]=0;
				}
			}
		printf("%d\n",cnt);
	}
	return 0;
}