#include <stdio.h>
#include <math.h>

int main(void) {
	int data_num,cur_data;
	double qr,rp,pq;
	int n,i;
	int p,q,r;
	int ap,aq,ar;
	double x[30];
	double y[30];
	double z[30];
	double nqr,nrp,npq;
	scanf("%d",&data_num);
	for(cur_data=0;cur_data<data_num;cur_data++) {
		scanf("%lf%lf%lf",&qr,&rp,&pq);
		scanf("%d",&n);
		for(i=0;i<n;i++)scanf("%lf%lf%lf",&x[i],&y[i],&z[i]);
		ap=aq=ar=0;
		for(p=0;p<n;p++) {
			for(q=0;q<n;q++) {
				if(p==q)continue;
				for(r=0;r<n;r++) {
					if(p==r || q==r)continue;
					nqr=sqrt((x[q]-x[r])*(x[q]-x[r])+
						(y[q]-y[r])*(y[q]-y[r])+
						(z[q]-z[r])*(z[q]-z[r]));
					nrp=sqrt((x[r]-x[p])*(x[r]-x[p])+
						(y[r]-y[p])*(y[r]-y[p])+
						(z[r]-z[p])*(z[r]-z[p]));
					npq=sqrt((x[p]-x[q])*(x[p]-x[q])+
						(y[p]-y[q])*(y[p]-y[q])+
						(z[p]-z[q])*(z[p]-z[q]));
					if(fabs(nrp/nqr-rp/qr)<=0.0001 &&
							fabs(npq/nrp-pq/rp)<=0.0001 &&
							fabs(nqr/npq-qr/pq)<=0.0001) {
						ap=p;aq=q;ar=r;
					}
				}
			}
		}
		printf("%d %d %d\n",ap+1,aq+1,ar+1);
	}
	return 0;
}