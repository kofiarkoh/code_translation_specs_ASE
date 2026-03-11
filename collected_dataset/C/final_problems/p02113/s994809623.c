#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define rep(i,l,r)for(int i=(l);i<(r);i++)

int c(const void*s,const void*t){return strcmp(s,t);}
char s[1010][40];
char t[40];
int f[1010];

int ans[1010],cnt;
int main(){
	int n,len;
	scanf("%d%d",&n,&len);
	rep(i,0,n)scanf(" %s",s[i]);
	qsort(s,n,40,c);
	
	rep(i,0,n)if(!f[i]){
		rep(j,0,len)t[j]=s[i][len-1-j];
		rep(j,i+1,n)if(!f[j]&&!strcmp(t,s[j])){
			ans[cnt++]=i;
			f[i]=f[j]=1;
			break;
		}
	}
	int flag=0;
	rep(i,0,n)if(!f[i]){
		rep(j,0,len)t[j]=s[i][len-1-j];
		if(!strcmp(s[i],t)){
			ans[cnt++]=i;
			flag=1;
			break;
		}
	}
	
	rep(i,0,cnt)printf("%s",s[ans[i]]);
	for(int i=cnt-1-flag;i>=0;i--)rep(j,0,len)putchar(s[ans[i]][len-1-j]);
	puts("");
}
