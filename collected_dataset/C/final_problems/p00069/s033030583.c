#include <stdio.h>

int accept(int m,int c,int n,int d,int amida[30][8]){
  int i;
  
  for(i = 0;i < d; i++){
    if(m != 1){
      if(amida[i][m-2] == 1){
	m--;
	continue;
      }
    }

    if(m != n){
      if(amida[i][m-1] == 1){
	m++;
	continue;
      }
    }
  }
  
  if(m == c){
    return 1;
  }else{
    return 0;
  }
}

int main(int argc,char* argv[]){
  int n,d,m,c;
  int amida[30][8];
  char str[10];
  int i;
  int end = 0;
  
  for(;;){
    int trace_m;
    end = 0;

    scanf("%d",&n);
    if(n == 0){break;}
  
    scanf("%d",&m);
    scanf("%d",&c);
    scanf("%d",&d);

    for(i = 0;i < d;i++){
      int j;
      scanf("%s",str);
      for(j = 0;j < (n-1);j++){
	amida[i][j] = str[j] - '0';
      }
    }

    if(accept(m,c,n,d,amida) == 1){
      printf("0\n");
      continue;
    }
    
    
    trace_m = m;
    for(i = 0;i < d;i++){
      if(trace_m != 1){
	if(amida[i][trace_m-2] == 1){
	  trace_m--;
	  continue;
	}
      }

      if(trace_m != n){
	if(amida[i][trace_m-1] == 1){
	  trace_m++;
	  continue;
	}
      }

      if(trace_m != 1){
	if(trace_m == 2 || amida[i][trace_m-3] == 0){
	  amida[i][trace_m-2] = 1;
	  if(accept(m,c,n,d,amida) == 1){
	    printf("%d %d\n",i+1,trace_m-1);
	    end = 1;
	  }
	  amida[i][trace_m-2] = 0;
	}
      }
      
      if(end == 1){
	break;
      }

      if(trace_m != n){
	if(trace_m == (n-1) || amida[i][trace_m] == 0){
	  amida[i][trace_m-1] = 1;
	  if(accept(m,c,n,d,amida) == 1){
	    printf("%d %d\n",i+1,trace_m);
	    end = 1;
	  }
	  amida[i][trace_m-1] = 0;
	}
      }
      
      if(end == 1){
	break;
      }
      
    }

    if(end == 1){
      continue;
    }

    printf("1\n");
  }
  return 0;
  
}