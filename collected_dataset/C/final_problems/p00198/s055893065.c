#include<stdio.h>
int main(void)
{
	int n,i,j,z;
	char c1[1001],c2[1001],c3[1001],c4[1001],c5[1001],c6[1001];
	scanf("%d",&n);
	while(n!=0){
	for(i=0;i<n;i++){
		scanf("%s %s %s %s %s %s",&c1[i],&c2[i],&c3[i],&c4[i],&c5[i],&c6[i]);
	}
	for(i=0;i<n;i++){
		z=0;
		for(j=0;j<n;j++){
		if(c1[i]==c1[j] && c2[i]==c2[j] && c3[i]==c3[j] && c4[i]==c4[j] && c5[i]==c5[j] && c6[i]==c6[j]){
			z=z+1;
		}
		else if(c4[i]==c4[j] && c1[i]==c1[j] && c3[i]==c3[j] && c6[i]==c6[j]){
			if(c5[i]==c5[j] && c2[i]==c2[j]){
				z=z+1;
			}
		}
		else if(c4[i]==c6[j] && c1[i]==c3[j] && c3[i]==c1[j] && c6[i]==c4[j]){
			if(c5[i]==c2[j] && c2[i]==c5[j]){
				z=z+1;
			}
		}
		else if(c6[i]==c6[j] && c3[i]==c3[j] && c1[i]==c1[j] && c4[i]==c4[j]){
			if(c5[i]==c5[j] && c2[i]==c2[j]){
				z=z+1;
			}
		}
		else if(c6[i]==c4[j] && c3[i]==c1[j] && c1[i]==c3[j] && c4[i]==c6[j]){
			if(c2[i]==c5[j] && c5[i]==c2[j]){
				z=z+1;
			}
		}
		else if(c2[i]==c2[j] && c1[i]==c1[j] && c5[i]==c5[j] && c6[i]==c6[j]){
			if(c4[i]==c4[j] && c3[i]==c3[j]){
				z=z+1;
			}
		}
		else if(c2[i]==c6[j] && c1[i]==c5[j] && c5[i]==c1[j] && c6[i]==c2[j]){
			if(c4[i]==c3[j] && c3[i]==c4[j]){
				z=z+1;
			}
		}
		else if(c6[i]==c6[j] && c5[i]==c5[j] && c1[i]==c1[j] && c2[i]==c2[j]){
			if(c3[i]==c3[j] && c4[i]==c4[j]){
				z=z+1;
			}
		}
		else if(c6[i]==c2[j] && c5[i]==c1[j] && c1[i]==c5[j] && c2[i]==c6[j]){
			if(c3[i]==c4[j] && c4[i]==c3[j]){
				z=z+1;
			}
		}
		}
	}
	printf("%d\n",z);
	scanf("%d",&n);
	}
	return 0;
}