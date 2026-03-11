#include <stdio.h>
#include <stdlib.h>
#include <string.h>
//using namespace std;

#define repl(i,a,b) for(i=(int)(a);i<(int)(b);i++)
#define rep(i,n) repl(i,0,n)

typedef long long ll;
typedef unsigned long long ull;

int int_sort( const void * a , const void * b ) {
  if( *( int * )a < *( int * )b ) {
    return -1;
  }
  else
  if( *( int * )a == *( int * )b ) {
    return 0;
  }
  return 1;
}

int stoi(char *s,int len){
	int res=0;
	int i;
	rep(i,len){
		res*=10;
		res+=s[i]-'0';
	}
	return res;
}

int cases;
char s[11111];

int dfs(int l,int r){
	int num=1;
	int i;
	repl(i,l+1,r-1){
		if(s[i]<'0'||s[i]>'9'){
			num=0;
			break;
		}
	}
	if(num){
		return stoi(s+l+1,r-l-2)/2+1;
	}else{
		int level=-1;
		int ll;
		int ns[11111];
		int idx=0;
		repl(i,l,r){
			if(s[i]=='['){
				if(level==0){
					ll=i;
				}
				level++;
			}else if(s[i]==']'){
				level--;
				if(level==0){
					//ns.pb(dfs(ll,i+1));
					ns[idx++]=dfs(ll,i+1);
				}
			}
		}
		//sort(all(ns));
		qsort((void *)ns,idx,sizeof(int),int_sort);
		int res=0;
		rep(i,idx/2+1){
			res+=ns[i];
		}
		return res;
	}
}

int main(){
	scanf("%d",&cases);
	while(cases--){
		scanf("%s",s);
		printf("%d\n",dfs(0,strlen(s)));
	}
	return 0;
}