#include <stdio.h>
#include <string.h>

int main()
{
	int i,j,k,l1,l2,l,f1,f2,c,flag;
	char s1[200],s2[200],buf[200];
	while(1){
		for(i=0;i<200;i++){
			s1[i] = s2[i] = buf[i] = '\0';
		}
		flag = c = 0;
		scanf("%s",s1);
		if( strcmp(s1,".") == 0 ) break;
		scanf("%s",s2);
		l1 = strlen(s1);
		l2 = strlen(s2);
		l = l1;
		if( l1 > l2 ){
			l = l2;
			strcpy( buf, s1 );
			strcpy( s1 , s2 );
			strcpy( s2 , buf);
		}
		for( i = 0, j = 0; i < l ; i++, j++){
			if( s1[i] != s2[j] ){
				if( flag == 1 ){
					f1 = f2 = 0;
					c++;
					for(k=0;k<l;k++){
						if( s1[i] == '"' ){
							flag = 0;
							f1 = 1;
							if( f1==1 && f2==1 ) break;
						}else{
							i++;
						}
						if( s2[j] == '"'){
							f1 = 1;
							if( f1 == 1 && f2==1 ) break;
						}else{
							j++;
						}
					}
				}else{ c = 2; break; }
			}else if( s1[i] == '"' ){
				flag = (flag+1) % 2;
			}
		};
		if( c == 0 && s2[j] == '\0' ) printf("IDENTICAL\n");
		else if( c == 1 ) printf("CLOSE\n");
		else printf("DIFFERENT\n");
	}
	return 0;
}
							
			
