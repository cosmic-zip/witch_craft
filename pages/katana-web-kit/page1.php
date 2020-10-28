<?php include_once 'header.php' ?>

<div class="container p-5">
    <h3>Hash</h3>
    <form action="" method="post">
    <label for="ps">Parse</label>
    <input type="number" name="ps" id="ps">
    <label for="type">Type</label>
    <select name="opt" id="type">
        <option value="md2">md2</option>
        <option value="md4">md4</option>
        <option value="md5">md5</option>
        <option value="sha1">sha1</option>
        <option value="sha256">sha256</option>
        <option value="sha384">sha384</option>
        <option value="sha512">sha512</option>
        <option value="ripemd128">ripemd128</option>
        <option value="ripemd160">ripemd160</option>
        <option value="ripemd256">ripemd256</option>
        <option value="ripemd320">ripemd320</option>
        <option value="whirlpool">whirlpool</option>
        <option value="tiger128">tiger128</option>
        <option value="tiger160">tiger160</option>
        <option value="tiger192">tiger192</option>
        <option value="tiger128">tiger128</option>
        <option value="tiger160">tiger160</option>
        <option value="tiger192">tiger192</option>
        <option value="snefru">snefru</option>
        <option value="gost">gost</option>
        <option value="adler32">adler32</option>
        <option value="crc32">crc32</option>
        <option value="crc32b">crc32b</option>
        <option value="haval128">haval128</option>
        <option value="haval160">haval160</option>
        <option value="haval192">haval192</option>
        <option value="haval224">haval224</option>
        <option value="haval256">haval256</option>
        <option value="haval128">haval128</option>
        <option value="haval160">haval160</option>
        <option value="haval192">haval192</option>
        <option value="haval224">haval224</option>
        <option value="haval256">haval256</option>
        <option value="haval128">haval128</option>
        <option value="haval160">haval160</option>
        <option value="haval192">haval192</option>
        <option value="haval224">haval224</option>
        <option value="haval25">haval25</option>
    </select>
    <label for="text">Text</label>
    <input type="text" id="text" name="text">
    <button class="btn btn-primary">Hash</button>
    </form>
</div>

<div class="container">
    <p class="p-4 m-4">
        <?php

                @$opt = $_POST['opt'];
                @$text = $_POST['text'];
                @$pas = $_POST['ps'];

                if($pas <= 1){
                  echo hash($opt, $text);
                }

                for($i=0; $i < $pas; $i++){
                    $text = hash($opt, $text);
                }
                echo $text;

        ?>
    </p>
</div>

<?php include_once 'footer.php' ?>
