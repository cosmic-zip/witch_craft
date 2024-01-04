import Image from 'next/image'
 
export default function WitchLogoBlack() {
  return (
    <Image
      src="/images/witch_logo_black.png"
      width={500}
      height={500}
      alt="Picture of the author"
      className='p-2'
    />
  )
}