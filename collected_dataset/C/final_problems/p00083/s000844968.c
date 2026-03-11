#include<stdio.h>
int main(){
  int a,b,c,d,t=0;
  char s;
  while(scanf("%d %d %d",&a,&b,&c)!=EOF){
    if(a<=1868 && b<=9 && c<8){
      printf("pre-meiji\n");
      d=1;
      t=1;
    }
    else if(a<=1912){
      if(a!=1912){
	printf("meiji ");
	a=a-1868+1;
	d=0;
	t=1;
      }
      else if(a==1912){
	if(b<7){
	  printf("meiji ");
	  a=a-1868+1;
	  d=0;
	  t=1;
	}
	else if(b>=7){
	  if(c<30){
	    printf("meiji ");
	    a=a-1868+1;
	    d=0;
	    t=1;
	  }
	}
      }
    }
    if(a<=1926 && t==0){
      if(a!=1926){
	printf("taisyo ");
	a=a-1912+1;
	d=0;
	t=1;
      }
      else if(a==1926){
	if(b<12){
	  printf("taisyo ");
	  a=a-1912+1;
	  d=0;
	  t=1;
	}
	else if(b==12){
	  if(c<25){
	    printf("taisyo ");
	    a=a-1912+1;
	    d=0;
	    t=1;
	  }
	  if(c>=25){
	    printf("showa ");
	    a=a-1926+1;
	    d=0;
	    t=1;
	  }
	}
      }
    }
    if(a<=1989 && t==0){
      if(a!=1989){
	printf("showa ");
	a=a-1926+1;
	d=0;
	t=1;
      }
      else if(a==1989){
	if(b==1){
	  if(c<8){
	    printf("showa ");
	    a=a-1926+1;
	    d=0;
	    t=1;
	  }
	}
      }
    }
    if(a>=1989 && t==0){
      if(a!=1989){
      printf("heisei ");
      a=a-1989+1;
      d=0;
    }
      else if(a==1989){
	if(b!=1){
	  printf("heisei ");
	  a=a-1989+1;
	  d=0;
	}
	else if(b==1){
	  if(c>=8){
	  printf("heisei ");
	  a=a-1989+1;
	  d=0;
	  }
	}
      }
    }
    if(d==0){
      printf("%d %d %d\n",a,b,c);
    }
  }
  return (0);
}