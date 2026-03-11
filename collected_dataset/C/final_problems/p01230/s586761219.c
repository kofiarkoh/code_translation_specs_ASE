#include<stdio.h>
#include<stdlib.h>

int main(void)
{
  int n,m,z;
  int a[50],b[50],c[50][2];
  int dp[50][2];
  int i,j,k;

  while(1){
    scanf("%d %d %d",&n,&m,&z);
    if(n==0 && m==0 && z==0)break;

    dp[0][1] = 2;
    for(i=1;i<n;i++)dp[i][1] = 0;

    for(i=0;i<m;i++)scanf("%d %d",&a[i],&b[i]);
    
    for(k=0;k<z;k+=2){
	for(i=0;i<n;i++){
	  dp[i][0] = 0;
	  c[i][0] = 0;
	}
	for(i=0;i<n;i++){
	  if(dp[i][1]>=1){
	    for(j=0;j<m;j++){
	      if(a[j]==i+1){
		dp[b[j]-1][0]+=1;
		c[j][0]=1;
	      }else if(b[j]==i+1){
		dp[a[j]-1][0]+=1;
		c[j][0]=1;
	      }
	    }
	  }
	  if(dp[i][1]==1){
	    for(j=0;j<m;j++){
	      if(c[j][1]==1){
		if(a[j]==i+1){
		  dp[b[j]-1][0]=0;
		}else if(b[j]==1+1){
		  dp[a[j]-1][0]=0;
		}
	      }
	    }
	  }  
	}
	k++;
	if(k==z)break;
	for(i=0;i<n;i++){
	  dp[i][1] = 0;
	  c[i][1] = 0;
	}
	for(i=0;i<n;i++){
	  if(dp[i][0]>=1){
	    for(j=0;j<m;j++){
	      if(a[j]==i+1){
		dp[b[j]-1][1]+=1;
		c[j][1]=1;
	      }else if(b[j]==i+1){
		dp[a[j]-1][1]+=1;
		c[j][1]=1;
	      }
	    }
	  }
	  if(dp[i][0]==1){
	    for(j=0;j<m;j++){
	      if(c[j][0]==1){
		if(a[j]==1+1){
		  dp[b[j]-1][1]=0;
		}else if(b[j]==1+1){
		  dp[a[j]-1][1]=0;
		}
	      }
	    }
	  }  
	}
      }
    
    if(z%2==0){
      if(dp[n-1][1]>=1)printf("yes\n");
      else printf("no\n");
    }else{
      if(dp[n-1][0]>=1)printf("yes\n");
      else printf("no\n");
    }
  }
  return 0;
}