#include <unistd.h>
#include <stdint.h>

int main(void) {
    static unsigned char inbuf[10000];
    static char outbuf[20000];

    ssize_t inlen = read(0, inbuf, sizeof(inbuf));
    if (inlen <= 0) {
        return 0;
    }

    unsigned char *p = inbuf;
    unsigned char *end = inbuf + inlen;

    /* Skip initial contiguous run of bytes >= '0' */
    while (p < end && *p >= (unsigned char)'0') {
        p++;
    }

    char *outp = outbuf;
    char *outend = outbuf + sizeof(outbuf);

    /* Process records only while current byte at record boundary is >= '0' */
    while (p < end && *p >= (unsigned char)'0') {
        int N = 0;
        int D = 0;

        /* Parse N: consume digits (bytes >= '0'), stop at first byte < '0', consume terminator */
        while (p < end && *p >= (unsigned char)'0') {
            N = N * 10 + (int)(*p - (unsigned char)'0');
            p++;
        }
        /* Consume terminator byte (undefined if missing) */
        if (p < end) p++;

        /* Parse D similarly */
        while (p < end && *p >= (unsigned char)'0') {
            D = D * 10 + (int)(*p - (unsigned char)'0');
            p++;
        }
        if (p < end) p++;

        int result;
        if (N & 1) {
            result = 127 * (N - 1) + D;
        } else {
            result = 127 * N - D;
        }

        /* Convert non-negative int to decimal, up to 10 digits, then newline */
        char tmp[10];
        int tlen = 0;

        if (result == 0) {
            if (outp < outend) *outp++ = '0';
        } else {
            /* Assumes result is non-negative per specification */
            while (result != 0) {
                int q = result / 10;
                int r = result - q * 10;
                tmp[tlen++] = (char)('0' + r);
                result = q;
                /* If more than 10 digits, behavior is undefined by spec; we do not guard. */
            }
            while (tlen-- > 0) {
                if (outp < outend) *outp++ = tmp[tlen];
            }
        }

        if (outp < outend) *outp++ = '\n';
    }

    /* Write accumulated output once at the end */
    (void)write(1, outbuf, (size_t)(outp - outbuf));
    return 0;
}
