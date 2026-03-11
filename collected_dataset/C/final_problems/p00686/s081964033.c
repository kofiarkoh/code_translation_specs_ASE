#include <stdio.h>
#define N 100
#define M 100
#define CMD 51

#define DEBUG 0

int main(void){
  char cmd[CMD];
  char buf[20];
  int n,m;
  int i,j,k;
  int flag;

  int x,y,t;

  while( 1 ){
    fgets(cmd,CMD,stdin);
    for( i=0; cmd[i]==' '||cmd[i]=='\t';i++ );
    for( j=0; '0'<=cmd[i]&&cmd[i]<='9' ; i++ )      buf[j++]=cmd[i];
    buf[j]='\0';
    sscanf(buf,"%d ",&m);
    for( ; cmd[i]==' '||cmd[i]=='\t';i++ );
    for( j=0; '0'<=cmd[i]&&cmd[i]<='9' ; i++ )      buf[j++]=cmd[i];
    sscanf(buf,"%d ",&n);

    if( m==0 && n==0 )  break;
    if( DEBUG==1 )  printf("field : %d %d \n",m,n);

    x=1;y=1;t=1;

    flag = 1;
    while( flag>0 ){
      fgets(cmd,CMD,stdin);
      if( DEBUG==1 )
	printf("%s| %d,%d t=%d\n",cmd,x,y,t);

	switch( cmd[0] ){
	case 'F':
	  for( i=7,j=0;cmd[i]!='\n';i++)
	    buf[j++]=cmd[i];
	  buf[j]='\0';
	  sscanf(buf,"%d ",&k);
	  switch(t){
	  case 1:   y+=k;   if( y>n ){  y=n;flag=2;  }	    break;
	  case 2:   x+=k;   if( x>m ){  x=m;flag=2;  }	    break;
	  case 3:   y-=k;   if( y<1 ){  y=1;flag=2;  }	    break;
	  case 4:   x-=k;   if( x<1 ){  x=1;flag=2;  }      break;
	  }
	  break;
	case 'B':
	  for( i=8,j=0;cmd[i]!='\n';i++)    buf[j++]=cmd[i];
	  buf[j]='\0';
	  sscanf(buf,"%d ",&k);
	  switch(t){
	  case 1:   y-=k;   if( y<1 ){  y=1;flag=2;   }	    break;
	  case 2:   x-=k;   if( x<1 ){  x=1;flag=2;   }	    break;
	  case 3:   y+=k;   if( y>n ){  y=n;flag=2;   }	    break;
	  case 4:   x+=k;   if( x>m ){  x=m;flag=2;   }	    break;
	  }
	  break;
	case 'R':
	  t++;	  if( t==5 )t=0;
	  break;
	case 'L':
	  t--;	  if( t==0 )t=4;
	  break;
	case 'S':
	  flag=0;
	  break;
	}
    }
    printf("%d %d\n",x,y);
  }
  return 0;
}