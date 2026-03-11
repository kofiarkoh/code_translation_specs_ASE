#include <stdio.h>

int main(void) {
	int l_total,l_ok;
	int d_total,d_ok;
	int m_total,m_ok;
	int t_h,t_m,zikan;
	int d_n,i;
	while(1) {
		scanf("%d",&d_n);
		if(d_n==0)break;
		l_total=l_ok=0;
		d_total=d_ok=0;
		m_total=m_ok=0;
		for(i=0;i<d_n;i++) {
			scanf("%d:%d%d",&t_h,&t_m,&zikan);
			if(t_h>=11 && t_h<15) {
				l_total++;
				zikan=(zikan+60-t_m)%60;
				if(zikan<=8)l_ok++;
			} else if(t_h>=18 && t_h<21) {
				d_total++;
				zikan=(zikan+60-t_m)%60;
				if(zikan<=8)d_ok++;
			} else if(t_h>=21 || t_h<2) {
				m_total++;
				zikan=(zikan+60-t_m)%60;
				if(zikan<=8)m_ok++;
			}
		}
		if(l_total>0)printf("lunch %d\n",l_ok*100/l_total);
		else puts("lunch no guest");
		if(d_total>0)printf("dinner %d\n",d_ok*100/d_total);
		else puts("dinner no guest");
		if(m_total>0)printf("midnight %d\n",m_ok*100/m_total);
		else puts("midnight no guest");
	}
	return 0;
}