#include<stdio.h>

int main(){
		int n,i,ans=0,tmp,num=0,j;
		long a[100000],flag[100000];
		scanf("%d",&n);
		for(i=0;i<n;i++){
				scanf("%d",&a[i]);
		}
		for(j=0;j<n;j++){
				tmp=0;
				int f=0;
				for(i=0;i<num;i++){
						if(flag[i]==a[j])f=1;
				}
				if(f!=1){
				for(i=0;i<n;i++){
								if(a[i]==a[j]){
										tmp++;
								}
				}
				if(tmp>a[j])ans+=tmp-a[j];
				else if(tmp<a[j])ans+=tmp;
				flag[num++]=a[j];
				}
		}
		printf("%d\n",ans);
		return 0;
}
