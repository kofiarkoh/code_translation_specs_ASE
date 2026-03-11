#include<stdio.h>
int main(void)
{
	int n,a,b,c,i;
    int	s[1000];
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%d",&a);
		scanf("%d",&b);
		scanf("%d",&c);
		if(a*a+b*b==c*c || b*b+c*c==a*a || a*a+c*c==b*b){
			s[i]=1;
		}
		else{
			s[i]=0;
	}
	}
	for(i=0;i<n;i++){
		if(s[i]==1){
			printf("YES\n");
	}
	    if(s[i]==0){
			printf("NO\n");
		}
	}
	return 0;
}
