#include<stdio.h>
#include<string.h>

char s[100010],t[100010];
int ans[30];
int main(){
	scanf("%s %s",s,t);
	int sn=strlen(s);
	int tn=strlen(t);
	if(tn==1){
		int cnt=0;
		for(int i=0;i<sn;i++)if(s[i]==t[0])cnt++;
		int ans=0;
		while(cnt>=(1<<ans)){
			cnt-=1<<ans;
			ans++;
		}
		printf("%d\n",ans);
		return 0;
	}
	int pos=0;
	for(int i=0;i<sn;i++){
		if(s[i]==t[ans[pos]]){
			ans[pos]++;
			if(pos==0&&ans[pos]==tn){
				while(ans[pos]==tn)ans[pos++]=0;
			}else pos=0;
		}
	}
	for(int i=0;i<30;i++)if(ans[i])pos=i;
	printf("%d\n",pos);
}
