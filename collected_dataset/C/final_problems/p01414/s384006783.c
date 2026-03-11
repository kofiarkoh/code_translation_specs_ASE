#include<stdio.h>
void f(int a){
	int i;
	for(i=16;i--;)printf("%d",(a>>i)%2);
	printf("\n");
}
int n,m,i,j,k,l,x,y,p,r,t=0,h[20],w[20],q[1<<17],d[1<<17]={1};
int main(){
	char s[20],c[5]={"RGB"};
	scanf("%d",&n);
	for(i=0;i<n;i++)scanf("%d %d",&h[i],&w[i]);
	for(i=0;i<16;i++)scanf(" %c",&s[i]);
	q[t=0]=0;
	for(r=1;r-t;t++){
		for(i=0;i<n;i++){
			for(j=0;j<3;j++){
				for(k=-h[i]+1;k<4;k++){
					for(l=-w[i]+1;l<4;l++){
						m=q[t];
						for(y=0;y<h[i];y++){
							for(x=0;x<w[i];x++){
								if(k+y<0||3<k+y||l+x<0||3<l+x)continue;
								p=(k+y)*4+l+x;
								m|=1<<p;
								if(s[p]-c[j])m-=1<<p;
							}
						}//f(m);
						if(d[m]==0)d[q[r++]=m]=d[q[t]]+1;
					}
				}//printf("\n%d\n\n",r);
			}
		}//printf("\n\n%d ",r);
		//f(q[t]);
	}
	//for(i=0;i<(1<<16);i++)printf("%d\n",d[i]);
	printf("%d\n",d[(1<<16)-1]-1);
	return 0;
}
