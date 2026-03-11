#include <stdio.h>
#include <string.h>

char ss[1010];
char tt[1010];
char*s,*t;
int ans[1010];
int tn;

void f(char*s){
	//この位置からtを引く
	for(int i=0;i<tn;i++)s[i]-=t[i]-'0';
	for(int i=tn-1;i>=0;i--)if(s[i]<'0')s[i]+=10,s[i-1]--;
}

int main(){
	scanf("%s %s",ss,tt);
	int sm=(ss[0]=='-'),tm=(tt[0]=='-');
	s=ss+sm,t=tt+tm;
	tn=strlen(t);
	while(1){
		int sn=strlen(s);
		if(sn<tn)break;
		if(sn==tn){
			if(strcmp(s,t)<0)break;
			f(s);
			ans[sn-tn]++;
		}else{
			int flag=strcmp(s,t)<0;
			f(s+flag);
			ans[sn-tn-flag]++;
		}
		while(*s=='0')s++;
	}
	int digit=0;
	for(int i=0;i<1010;i++)if(ans[i])digit=i;
	if((sm^tm)&&(digit>0||ans[0]!=0))printf("-");
	for(int i=digit;i>=0;i--)printf("%d",ans[i]);
	puts("");
	
}
