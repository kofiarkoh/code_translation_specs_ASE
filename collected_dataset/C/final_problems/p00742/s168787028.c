#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <assert.h>

#define N 13
#define L 9
#define DIG 10
#define VCMAX 12
#define CMAX 128
#define TRUE 1
#define FALSE 0

int n;
int cnum;
char m[N][L];
char f[CMAX];
int non_zero[CMAX];
char vc[VCMAX];
char onenum[VCMAX];
int used[DIG];
int mlen[N];
int onum;
int g_cnt;

void ResetGlobal(){
  int i,j;
  n = onum = cnum = 0;
  for(i = 0; i < CMAX;++i){
    f[i]=0;
    non_zero[i]=FALSE;
  }
  for(i = 0; i < DIG; ++i){
    used[i] = FALSE;
  }
  for(i = 0; i < VCMAX; ++i){
    vc[i]=0;
    onenum[i]=0;
  }
  for(i = 0; i < N; ++i){
    mlen[i]=0;
    for(j = 0; j < L; ++j){
      m[i][j] = 0;
    }
  }
}
int dfs(int depth){
  int i,j;
  int ret = 0;

  int t=0;
  int cnt=0;
  /*
  for(i = 0; i < 128; ++i){
    if(f[i]>='0'){
      printf("map : %c -> %c\n", i, f[i]);
    }
  }
  putchar('\n');
  */
  

  
  for(i = 0; i < onum; ++i){
    if(f[onenum[i]]>='0'){++cnt;}
  }
  if(cnt==onum){
    /*
      for(i = 0; i < onum; ++i){
      printf("map : %c -> %c\n", onenum[i], f[onenum[i]]);
      }
    */    
    for(i=0;i<n-1;++i){
      t+=f[m[i][mlen[i]-1]]-'0';
    }
    /*
    for(i = 0;i < n;++i){
      for(j=0;j<mlen[i];++j){
	printf("%c",m[i][j]);
      }
      putchar('\n');
    }
    */
    if(t%10!=f[m[n-1][mlen[n-1]-1]]-'0'){
      return 0;
    }
  }

  if(depth==cnum){
    int sum=0;
   
    for(i = 0; i < n; ++i){
      char v[L]={0,};
      for(j = 0; j < mlen[i]; ++j){
	v[j] = f[ m[i][j] ];
      }
      if(i==n-1){
	if(sum==atoi(v)){
	  /*
	  for(i = 0; i < n; ++i){
	    for(j = 0; j < mlen[i]; ++j){
	      printf("%c",f[m[i][j]]);
	    }
	    putchar('\n');
	  }
	  printf("%s %d == %d\n",v,atoi(v),sum);
	  */
	  ++g_cnt;
	  return 1;
	}
	return 0;
      }

      sum+=atoi(v);
    }
    //putchar('\n');
  }
  for(i = 0; i < DIG; ++i){
    if( !used[i] ){
      if(i==0&&non_zero[vc[depth]] )continue;
      used[i]=TRUE;
      f[vc[depth]]=i+'0';
      ret += dfs(depth+1);
      f[vc[depth]]=0;
      used[i]=FALSE;
    }
  }
  return ret;
}

int main()
{
  int i,j,k;
  while(TRUE){
    int ans = 0;

    ResetGlobal();
    scanf("%d",&n);
    if(n==0)break;

    for(i = 0; i < n; ++i){
      char buf[CMAX];
      scanf("%s",buf);
      mlen[i]=strlen(buf);
      int len = strlen(buf);
      for(j = 0; j < len; ++j){
	m[i][j] = buf[j];
	int bFound=FALSE;
	if(j==len-1){
	  for(k=0;k<onum;++k){
	    if(onenum[k]==m[i][j]){bFound=TRUE;break;}
	  }
	  if(!bFound){
	    onenum[onum++]=m[i][j];
	  }
	}
	bFound=FALSE;
	for(k = 0; k < cnum; ++k){
	  if(vc[k]==m[i][j]){bFound=TRUE;break;}
	}
	if(!bFound)vc[cnum++]=m[i][j];
	if(j==0&&len>1)non_zero[buf[j]]=TRUE;
      }
    }

    /*
    for(i=0;i<VCMAX;++i){
      printf("VC[%d] = %c\n", i, vc[i]);
    }
    for(i=0;i<CMAX;++i){
      if(non_zero[i]){
	printf("NON_ZERO : %c\n", i);
      }
    }
    for(i=0;i<onum;++i){
      printf("ONE_DIG[%d] = %c\n", i, onenum[i]);
    }
    for(i = 0; i < n;++i){
      printf("len[i] = %d\n", mlen[i]);
    }
    */
    
    g_cnt=0;
    ans = dfs(0);
    printf("%d\n",ans);
  }
  return 0;
}